//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1013/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1013<F: Float>(t1307: F, t6562: F, t22914: F, t34787: F, t34535: F, t487: F, t492: F, t1337: F, t6454: F, t137463: F, t137471: F, t1564: F, t25618: F, t25856: F, t32016: F, t32375: F, t32423: F, t3266: F, t34366: F, t34614: F, t379: F, t5495: F, t5501: F, t5624: F, t6414: F, t8411: F, t91493: F) -> (F, F) {
    let t144733 = t1307 * t6562;
    let t144738 = t22914 * t34787;
    let t144744 = t34535 * t487;
    let t144745 = t144744 * t492;
    let t144752 = t6454 * t1337;
    let t144763 = -t5501 * t1564 * t144733 * t379 / F::new(9.0) + t144738 / F::new(27.0) + t6414 * t32375 / F::new(6.0) + t34614 * t5624 / F::new(6.0) - F::new(2.0) * t144745 + F::new(2.0) * t5501 * t8411 * t32423 * t3266 + t5495 * t34366 - t5501 * t1564 * t144752 * t379 / F::new(9.0) - F::new(24.0) * t91493 * t25856 + t137463 / F::new(9.0) - t137471 / F::new(18.0) - t32016 * t25618 / F::new(27.0);
    (t144745, t144763)
}
