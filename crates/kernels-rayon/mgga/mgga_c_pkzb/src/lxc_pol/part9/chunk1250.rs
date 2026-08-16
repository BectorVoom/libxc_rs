//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1250/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1250(t1137: f64, t17852: f64, t18091: f64, t18094: f64, t18097: f64, t18103: f64, t2020: f64, t2104: f64, t21448: f64, t21841: f64, t21843: f64, t21852: f64, t21863: f64, t21867: f64, t21870: f64, t287: f64, t2900: f64, t2903: f64, t2922: f64, t302: f64, t5635: f64, t5984: f64, t7642: f64) -> f64 {
    let t21872 = t18091 / 36.0_f64 - 0.45732285992607719436e-2_f64 * t21841 - 0.21437009059034868486e-3_f64 * t2922 * t302 * t2900 * t21843 + 0.43445671692977333464e-1_f64 * t2020 * t21448 * t2903 + 0.25724410870841842184e-2_f64 * t21852 - 0.51448821741683684368e-2_f64 * t2104 * t17852 * t1137 * t287 * t5635 + 0.85748036236139473944e-3_f64 * t18094 - 0.42874018118069736972e-3_f64 * t18097 + t21863 - 0.20579528696673473747e-1_f64 * t5984 * t7642 + 0.91464571985215438873e-2_f64 * t18103 - 0.27439371595564631662e-1_f64 * t21867 - 0.85748036236139473945e-3_f64 * t21870;
    t21872
}
