//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1141/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1141<F: Float>(t2096: F, t7692: F, t287: F, t5913: F, t17848: F, t2104: F, t7641: F, t17867: F, t2932: F, t7607: F, t7784: F, t2945: F, t2947: F, t5939: F, t1137: F, t17852: F, t18091: F, t18094: F, t18097: F, t18103: F, t2020: F, t21448: F, t2900: F, t2903: F, t2922: F, t302: F, t5635: F, t5984: F, t7642: F) -> (F, F) {
    let t21841 = t2096 * t7692;
    let t21843 = t5913 * t287;
    let t21852 = t2104 * t17848 * t7641;
    let t21862 = t2104 * t17867 * t2932;
    let t21863 = 0.28582678745379824648e-3 * t21862;
    let t21867 = t7607 * t7784;
    let t21870 = t2945 * t5939 * t2947;
    let t21872 = t18091 / 36.0 - 0.45732285992607719436e-2 * t21841 - 0.21437009059034868486e-3 * t2922 * t302 * t2900 * t21843 + 0.43445671692977333464e-1 * t2020 * t21448 * t2903 + 0.25724410870841842184e-2 * t21852 - 0.51448821741683684368e-2 * t2104 * t17852 * t1137 * t287 * t5635 + 0.85748036236139473944e-3 * t18094 - 0.42874018118069736972e-3 * t18097 + t21863 - 0.20579528696673473747e-1 * t5984 * t7642 + 0.91464571985215438873e-2 * t18103 - 0.27439371595564631662e-1 * t21867 - 0.85748036236139473945e-3 * t21870;
    (t21843, t21872)
}
