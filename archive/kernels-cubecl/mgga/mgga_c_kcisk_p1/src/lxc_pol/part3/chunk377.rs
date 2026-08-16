//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 377/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk377<F: Float>(t695: F, t708: F, t1060: F, t1876: F, t574: F, t1648: F, t706: F, t682: F, t707: F, t1824: F, t1421: F, t1689: F, t1875: F, t456: F, t604: F) -> (F, F, F, F, F, F, F, F, F) {
    let t1877 = t708 * t695;
    let t1879 = t1876 * t1877 * t1060;
    let t1882 = t574 * t708;
    let t1883 = t1882 * t1648;
    let t1884 = t706 * t1883;
    let t1887 = t707 * t682;
    let t1888 = t1887 * t1824;
    let t1889 = t706 * t1888;
    let t1894 = t1875 + F::cast_from(0.65704296666666666667e-3_f64) * t1421 * t1879 + F::cast_from(0.1478346675e-2_f64) * t456 * t1884 - F::cast_from(0.98556445e-3_f64) * t456 * t1889 - F::cast_from(4.0_f64) * t604 * t1689;
    (t1877, t1879, t1882, t1883, t1884, t1887, t1888, t1889, t1894)
}
