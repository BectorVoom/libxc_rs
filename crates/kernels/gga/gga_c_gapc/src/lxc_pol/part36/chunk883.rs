//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 883/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk883<F: Float>(t11466: F, t11469: F, t11471: F, t11475: F, t11477: F, t11481: F, t11486: F, t11490: F, t11493: F, t11497: F, t11501: F, t11504: F, t11506: F, t11510: F, t11515: F, t11520: F, t11524: F, t11527: F, t11529: F) -> (F,) {
    let t12410 = 0.63350674672043801542e-5 * t11466 - 0.13506074236995523433e-5 * t11469 - 0.13506074236995523433e-5 * t11471 - 0.80045999977926802215e-7 * t11475 - 0.80192315782160920384e-6 * t11477 + 0.33816362383187442027e-5 * t11481 + 0.4637672555408563478e-4 * t11486 - 0.9275345110817126956e-4 * t11490 + 0.9275345110817126956e-4 * t11493 + 0.77294542590142724635e-6 * t11497 - 0.1374296967252737644e-5 * t11501 + 0.45020247456651744776e-7 * t11504 + 0.45020247456651744777e-6 * t11506 - 0.6070699179094394313e-6 * t11510 + 0.10793703140429833089e-5 * t11515 + 0.98332751566569010433e-8 * t11520 + 0.50603841145833333338e-5 * t11524 + 0.50603841145833333338e-5 * t11527 + 0.3243554543208642639e-2 * t11529;
    (t12410,)
}
