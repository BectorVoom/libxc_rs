//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1175/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1175<F: Float>(t10249: F, t6305: F, t2268: F, t31585: F, t426: F, t535: F, t10119: F, t10232: F, t10590: F, t1063: F, t1306: F, t1324: F, t1358: F, t1365: F, t31652: F, t31655: F, t31660: F, t31662: F, t31672: F, t31674: F, t31679: F, t3371: F, t3808: F, t3822: F, t3833: F, t448: F) -> F {
    let t31681 = F::cast_from(0.68292015925622759036e0_f64) * t6305 * t10249;
    let t31685 = F::cast_from(0.56910013271352299198e-1_f64) * t2268 * t535 * t31585 * t426;
    let t31686 = -t31652 + F::cast_from(0.63233348079280332442e-2_f64) * t3808 * t10232 + F::cast_from(0.63233348079280332442e-2_f64) * t1358 * t1365 * t31655 - t31660 + t31662 - F::cast_from(0.56910013271352299198e-1_f64) * t3833 * t10119 - F::cast_from(0.56910013271352299198e-1_f64) * t1063 * t10590 * t448 - F::cast_from(0.28455006635676149599e-1_f64) * t1063 * t3371 * t1306 - t31672 - t31674 + F::cast_from(0.56910013271352299198e-1_f64) * t3822 * t3371 * t1324 + t31679 + t31681 + t31685;
    t31686
}
