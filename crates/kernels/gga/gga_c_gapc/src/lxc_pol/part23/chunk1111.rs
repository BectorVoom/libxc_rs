//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 1111/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk1111<F: Float>(t11872: F, t9990: F, t11356: F, t28472: F, t9574: F, t33770: F, t33772: F, t33774: F, t33777: F, t33779: F, t33784: F, t33787: F, t33789: F, t33791: F) -> F {
    let t33793 = t11872 * t9990;
    let t33796 = t9574 * t11356 * t28472;
    let t33798 = -F::new(0.52838066223730378166e-7) * t33770 - F::new(0.20010856351627032588e-7) * t33772 - F::new(0.20047434126173032506e-6) * t33774 + F::new(0.33147827249531850014e-7) * t33777 - F::new(0.28985453471303521737e-5) * t33779 - F::new(0.96681162811134562541e-9) * t33784 + F::new(0.1422820120100248667e-7) * t33787 + F::new(0.17391272082782113042e-4) * t33789 - F::new(0.21102562238076876322e-7) * t33791 + F::new(0.16882049790461501058e-6) * t33793 + F::new(0.10551281119038438161e-7) * t33796;
    t33798
}
