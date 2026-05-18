//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 981/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk981<F: Float>(t11863: F, t11865: F, t11867: F, t11870: F, t11873: F, t11879: F, t11882: F, t11885: F, t11890: F, t11893: F, t11895: F, t11898: F, t11900: F, t11903: F, t11906: F, t11908: F, t11911: F, t11914: F, t11919: F) -> F {
    let t11921 = -F::new(0.10120442708333333334e-4) * t11863 - F::new(0.10120442708333333334e-4) * t11865 + F::new(0.54106179813099907243e-4) * t11867 + F::new(0.76936424826321944924e-9) * t11870 + F::new(0.16882049790461501058e-6) * t11873 + F::new(0.50551591594011046914e-6) * t11879 - F::new(0.45289771048911752714e-7) * t11882 - F::new(0.26419033111865189083e-7) * t11885 + F::new(0.15837668668010950386e-5) * t11890 - F::new(0.16882049790461501058e-6) * t11893 - F::new(0.16882049790461501058e-6) * t11895 - F::new(0.10005428175813516294e-7) * t11898 - F::new(0.10023717063086516253e-6) * t11900 + F::new(0.45289771048911752714e-7) * t11903 - F::new(0.90579542097823505428e-7) * t11906 + F::new(0.90579542097823505428e-7) * t11908 - F::new(0.52756405595192190805e-8) * t11911 - F::new(0.10551281119038438161e-7) * t11914 + F::new(0.19645612283222543108e-8) * t11919;
    t11921
}
