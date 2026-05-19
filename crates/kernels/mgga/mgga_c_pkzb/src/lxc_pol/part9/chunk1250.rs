//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1250/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1250<F: Float>(t1137: F, t17852: F, t18091: F, t18094: F, t18097: F, t18103: F, t2020: F, t2104: F, t21448: F, t21841: F, t21843: F, t21852: F, t21863: F, t21867: F, t21870: F, t287: F, t2900: F, t2903: F, t2922: F, t302: F, t5635: F, t5984: F, t7642: F) -> F {
    let t21872 = t18091 / F::new(36.0) - F::cast_from(0.45732285992607719436e-2_f64) * t21841 - F::cast_from(0.21437009059034868486e-3_f64) * t2922 * t302 * t2900 * t21843 + F::cast_from(0.43445671692977333464e-1_f64) * t2020 * t21448 * t2903 + F::cast_from(0.25724410870841842184e-2_f64) * t21852 - F::cast_from(0.51448821741683684368e-2_f64) * t2104 * t17852 * t1137 * t287 * t5635 + F::cast_from(0.85748036236139473944e-3_f64) * t18094 - F::cast_from(0.42874018118069736972e-3_f64) * t18097 + t21863 - F::cast_from(0.20579528696673473747e-1_f64) * t5984 * t7642 + F::cast_from(0.91464571985215438873e-2_f64) * t18103 - F::cast_from(0.27439371595564631662e-1_f64) * t21867 - F::cast_from(0.85748036236139473945e-3_f64) * t21870;
    t21872
}
