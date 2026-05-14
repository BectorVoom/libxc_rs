//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 778/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk778<F: Float>(t34822: F, t7369: F, t7239: F, t7366: F, t2112: F, t34808: F, t28: F, t5890: F, t32924: F, t9073: F, t925: F, t5899: F, t34817: F, t9236: F, t1369: F, t1039: F, t7339: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t34823 = t7369 * t34822;
    let t34825 = t7366 * t7239 * t34823;
    let t34827 = t2112 * t34808;
    let t34829 = t5890 * t28 * t34827;
    let t34832 = t9073 * t32924 * t925;
    let t34833 = t5899 * t34832;
    let t34835 = t9236 * t34817;
    let t34837 = t1369 * t28 * t34835;
    let t34839 = t2112 * t34822;
    let t34841 = t1369 * t28 * t34839;
    let t34843 = t7339 * t1039;
    (t34823, t34825, t34827, t34829, t34832, t34833, t34835, t34837, t34839, t34841, t34843)
}
