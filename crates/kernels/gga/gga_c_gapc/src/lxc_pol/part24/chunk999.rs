//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 999/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk999<F: Float>(t11466: F, t11469: F, t11471: F, t11475: F, t11477: F, t11481: F, t11486: F, t11490: F, t11493: F, t11497: F, t11501: F, t11504: F, t11506: F, t11510: F, t11515: F, t11520: F, t11524: F, t11527: F, t11529: F) -> F {
    let t12410 = F::cast_from(0.63350674672043801542e-5_f64) * t11466 - F::cast_from(0.13506074236995523433e-5_f64) * t11469 - F::cast_from(0.13506074236995523433e-5_f64) * t11471 - F::cast_from(0.80045999977926802215e-7_f64) * t11475 - F::cast_from(0.80192315782160920384e-6_f64) * t11477 + F::cast_from(0.33816362383187442027e-5_f64) * t11481 + F::cast_from(0.4637672555408563478e-4_f64) * t11486 - F::cast_from(0.9275345110817126956e-4_f64) * t11490 + F::cast_from(0.9275345110817126956e-4_f64) * t11493 + F::cast_from(0.77294542590142724635e-6_f64) * t11497 - F::cast_from(0.1374296967252737644e-5_f64) * t11501 + F::cast_from(0.45020247456651744776e-7_f64) * t11504 + F::cast_from(0.45020247456651744777e-6_f64) * t11506 - F::cast_from(0.6070699179094394313e-6_f64) * t11510 + F::cast_from(0.10793703140429833089e-5_f64) * t11515 + F::cast_from(0.98332751566569010433e-8_f64) * t11520 + F::cast_from(0.50603841145833333338e-5_f64) * t11524 + F::cast_from(0.50603841145833333338e-5_f64) * t11527 + F::cast_from(0.3243554543208642639e-2_f64) * t11529;
    t12410
}
