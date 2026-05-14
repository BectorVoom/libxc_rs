//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1104/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1104<F: Float>(t11029: F, t2922: F, t774: F, t11033: F, t2104: F, t5974: F, t300: F, t3651: F, t11038: F, t10994: F, t2899: F, t11024: F, t11043: F, t10932: F, t11025: F, t11070: F, t11088: F, t17852: F, t2105: F, t2106: F, t21468: F, t25485: F, t2901: F, t29754: F, t29775: F, t302: F, t5965: F, t759: F, t761: F, t7725: F, t7742: F, t9282: F, t9314: F, t9319: F) -> (F, F, F, F, F) {
    let t29908 = t2922 * t774 * t11029;
    let t29911 = t2104 * t5974 * t11033;
    let t29918 = t300 * t3651;
    let t29928 = t2104 * t5974 * t11038;
    let t29950 = t2899 * t774 * t10994;
    let t29953 = t2922 * t5974 * t11024;
    let t29956 = t2899 * t5974 * t11043;
    let t29970 = -0.42874018118069736972e-3 * t25485 - 0.77173232612525526552e-2 * t21468 * t302 * t29775 * t9319 - 0.51448821741683684368e-2 * t2104 * t17852 * t10932 * t759 * t761 - 0.42874018118069736972e-3 * t2104 * t2105 * t11088 * t2106 - 0.68598428988911579157e-2 * t7725 * t11025 + 0.85748036236139473947e-3 * t29950 + 0.85748036236139473947e-3 * t29953 - 0.17149607247227894789e-2 * t29956 + 0.25724410870841842184e-2 * t7742 * t2105 * t11070 * t5965 - 0.38586616306262763276e-2 * t7742 * t302 * t9282 * t9314 + 0.42874018118069736972e-3 * t2899 * t302 * t29754 * t2901;
    (t29908, t29911, t29918, t29928, t29970)
}
