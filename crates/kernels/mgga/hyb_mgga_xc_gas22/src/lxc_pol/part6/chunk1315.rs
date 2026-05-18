//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1315/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1315<F: Float>(t4139: F, t6564: F, t2189: F, t6562: F, t10625: F, t10626: F, t10631: F, t10667: F, t10672: F, t20730: F, t20741: F, t20744: F, t2291: F, t2292: F, t2307: F, t2315: F, t2322: F, t3430: F, t4180: F, t4193: F, t4214: F, t6640: F, t6737: F, t856: F, t8600: F, t8601: F, t8608: F, t8743: F) -> (F, F) {
    let t28741 = t4139 * t6564;
    let t28744 = F::new(0.51726012919273400301e3) * t6562 * t28741 * t2189;
    let t28779 = t28744 - F::new(0.91082604192152556044e5) * t856 * t20741 * t4180 * t20744 * t2291 + F::new(0.10389515463408878255e3) * t856 * t6640 * t4193 * t2315 - F::new(0.35089341735807877242e1) * t856 * t4214 * t2307 - F::new(0.20508037716432813316e4) * t2322 * t10631 + F::new(0.46785788981077169656e1) * t3430 * t8743 + F::new(0.10389515463408878255e3) * t856 * t10667 * t6737 + F::new(0.12304822629859687989e5) * t856 * t20730 * t4180 * t8600 + F::new(0.23392894490538584828e1) * t2322 * t10672 - F::new(0.34631718211362927517e2) * t3430 * t8608 - F::new(0.20508037716432813315e4) * t3430 * t8601 - F::new(0.35089341735807877242e1) * t856 * t10625 * t2292 - F::new(0.34631718211362927518e2) * t2322 * t10626;
    (t28744, t28779)
}
