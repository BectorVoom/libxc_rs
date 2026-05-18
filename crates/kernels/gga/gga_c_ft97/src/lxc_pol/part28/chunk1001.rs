//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1001/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1001<F: Float>(t34614: F, t5498: F, t31998: F, t6414: F, t1286: F, t1337: F, t136059: F, t25528: F, t25533: F, t25584: F, t25602: F, t28: F, t3103: F, t32002: F, t32016: F, t32054: F, t32338: F, t32385: F, t32403: F, t5507: F, t5510: F, t6461: F, t7168: F, t7218: F) -> F {
    let t144416 = t34614 * t5498;
    let t144420 = t6414 * t31998;
    let t144442 = -t6414 * t32002 / F::new(3.0) + t32016 * t25602 / F::new(9.0) - t144416 / F::new(18.0) + t25584 * t7218 / F::new(3.0) - t144420 / F::new(18.0) + t32054 * t6461 / F::new(6.0) - t34614 * t5510 / F::new(3.0) - t25584 * t7168 / F::new(3.0) - F::new(2.0) / F::new(3.0) * t1286 * t28 * t5507 * t1337 * t3103 + t1286 * t28 * t32338 * t25533 + t6414 * t32403 - F::new(2.0) / F::new(3.0) * t1286 * t28 * t25528 * t32385 - t136059 / F::new(18.0);
    t144442
}
