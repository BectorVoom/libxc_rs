//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 933/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk933<F: Float>(t13296: F, t4342: F, t13301: F, t4339: F, t8675: F, t4343: F, t4335: F, t12143: F, t14421: F, t14423: F, t14426: F, t14429: F, t14431: F, t14434: F, t14439: F, t14442: F, t14445: F, t14448: F, t14451: F, t14455: F, t14460: F, t14464: F, t14468: F, t2265: F, t3628: F, t631: F) -> F {
    let t14471 = t4342 * t13296;
    let t14474 = t4342 * t13301;
    let t14478 = F::new(4.0) / F::new(9.0) * t8675 * t4339;
    let t14480 = F::new(4.0) / F::new(9.0) * t8675 * t4343;
    let t14482 = F::new(2.0) / F::new(27.0) * t8675 * t4335;
    let t14483 = t14421 + t14423 + F::new(6.0) * t631 * t14426 + F::new(13.0) / F::new(9.0) * t14429 + F::new(5.0) / F::new(27.0) * t14431 - F::new(3.0) / F::new(2.0) * t631 * t14434 - F::new(3.0) * t631 * t14439 - t3628 * t14442 / F::new(3.0) + F::new(5.0) / F::new(9.0) * t14445 - t14448 - F::new(2.0) / F::new(3.0) * t2265 * t14451 - t2265 * t14455 / F::new(3.0) - t2265 * t14460 / F::new(9.0) - t2265 * t14464 / F::new(3.0) + F::new(4.0) / F::new(3.0) * t12143 * t14468 - t2265 * t14471 / F::new(3.0) + F::new(4.0) / F::new(3.0) * t12143 * t14474 + t14478 + t14480 - t14482;
    t14483
}
