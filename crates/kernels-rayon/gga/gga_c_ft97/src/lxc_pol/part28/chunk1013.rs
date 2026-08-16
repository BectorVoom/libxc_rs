//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1013/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1013(t1307: f64, t6562: f64, t22914: f64, t34787: f64, t34535: f64, t487: f64, t492: f64, t1337: f64, t6454: f64, t137463: f64, t137471: f64, t1564: f64, t25618: f64, t25856: f64, t32016: f64, t32375: f64, t32423: f64, t3266: f64, t34366: f64, t34614: f64, t379: f64, t5495: f64, t5501: f64, t5624: f64, t6414: f64, t8411: f64, t91493: f64) -> (f64, f64) {
    let t144733 = t1307 * t6562;
    let t144738 = t22914 * t34787;
    let t144744 = t34535 * t487;
    let t144745 = t144744 * t492;
    let t144752 = t6454 * t1337;
    let t144763 = -t5501 * t1564 * t144733 * t379 / 9.0_f64 + t144738 / 27.0_f64 + t6414 * t32375 / 6.0_f64 + t34614 * t5624 / 6.0_f64 - 2.0_f64 * t144745 + 2.0_f64 * t5501 * t8411 * t32423 * t3266 + t5495 * t34366 - t5501 * t1564 * t144752 * t379 / 9.0_f64 - 24.0_f64 * t91493 * t25856 + t137463 / 9.0_f64 - t137471 / 18.0_f64 - t32016 * t25618 / 27.0_f64;
    (t144745, t144763)
}
