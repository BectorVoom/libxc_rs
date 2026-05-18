//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 870/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk870<F: Float>(t34817: F, t9236: F, t1369: F, t28: F, t2112: F, t34822: F, t1039: F, t7339: F, t586: F, t5890: F, t32896: F, t32923: F, t34811: F, t34815: F, t34820: F, t34825: F, t34829: F, t34833: F) -> (F, F, F, F, F, F, F, F) {
    let t34835 = t9236 * t34817;
    let t34837 = t1369 * t28 * t34835;
    let t34839 = t2112 * t34822;
    let t34841 = t1369 * t28 * t34839;
    let t34843 = t7339 * t1039;
    let t34844 = t586 * t34843;
    let t34846 = t5890 * t28 * t34844;
    let t34848 = F::new(3.0) / F::new(2.0) * t34811 + t32896 + F::new(2.0) / F::new(3.0) * t34815 + F::new(4.0) * t34820 - F::new(2.0) * t34825 - t34829 / F::new(2.0) - t32923 - t34833 / F::new(3.0) - F::new(3.0) * t34837 + F::new(2.0) * t34841 + t34846 / F::new(4.0);
    (t34835, t34837, t34839, t34841, t34843, t34844, t34846, t34848)
}
