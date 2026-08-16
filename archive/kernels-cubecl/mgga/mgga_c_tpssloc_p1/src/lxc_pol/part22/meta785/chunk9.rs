//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2712/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2712<F: Float>(t1449: F, t5484: F, t100: F, t103: F, t12795: F, t1447: F, t19514: F, t19518: F, t19522: F, t19525: F, t19526: F, t2: F, t20331: F, t20338: F, t2219: F, t2349: F, t4059: F, t4064: F, t45460: F, t45707: F, t5475: F, t5480: F, t55491: F, t584: F, t662: F, t75649: F) -> F {
    let t75676 = t1449 * t5484;
    let t75694 = -F::cast_from(200.0_f64) / F::cast_from(9.0_f64) * t5475 * t4064 + F::cast_from(50.0_f64) / F::cast_from(27.0_f64) * t1447 * t19514 + F::cast_from(100.0_f64) / F::cast_from(9.0_f64) * t55491 * t19518 - F::cast_from(50.0_f64) / F::cast_from(9.0_f64) * t1447 * t19522 - F::cast_from(25.0_f64) / F::cast_from(3.0_f64) * t1447 * t19526 + F::cast_from(40.0_f64) / F::cast_from(81.0_f64) * t100 * t45460 * t20331 * t662 + F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t45707 * t5480 * t2 * t584 - F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t45707 * t75676 * t662 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t12795 * t2219 * t5484 + F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t100 * t4059 * t19525 + F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t100 * t2349 * t20338 * t662 - F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t100 * t103 * t75649;
    t75694
}
