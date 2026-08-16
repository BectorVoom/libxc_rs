//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 377/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk377(t695: f64, t708: f64, t1060: f64, t1876: f64, t574: f64, t1648: f64, t706: f64, t682: f64, t707: f64, t1824: f64, t1421: f64, t1689: f64, t1875: f64, t456: f64, t604: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t1877 = t708 * t695;
    let t1879 = t1876 * t1877 * t1060;
    let t1882 = t574 * t708;
    let t1883 = t1882 * t1648;
    let t1884 = t706 * t1883;
    let t1887 = t707 * t682;
    let t1888 = t1887 * t1824;
    let t1889 = t706 * t1888;
    let t1894 = t1875 + 0.65704296666666666667e-3_f64 * t1421 * t1879 + 0.1478346675e-2_f64 * t456 * t1884 - 0.98556445e-3_f64 * t456 * t1889 - 4.0_f64 * t604 * t1689;
    (t1877, t1879, t1882, t1883, t1884, t1887, t1888, t1889, t1894)
}
