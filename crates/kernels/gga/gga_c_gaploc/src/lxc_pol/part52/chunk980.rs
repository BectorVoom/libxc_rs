//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 980/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk980<F: Float>(t12161: F, t13045: F, t14385: F, t1445: F, t2197: F, t2949: F, t44117: F, t44133: F, t45913: F, t45915: F, t45922: F, t45931: F, t45933: F, t45939: F, t45946: F, t45947: F, t45950: F, t45953: F, t45958: F, t47496: F, t47500: F, t47517: F, t47555: F, t47558: F, t813: F) -> F {
    let t50302 = t45913 + F::new(0.11916829983950142223e0) * t47517 + t45915 - F::new(0.63904876589867916127e-1) * t44117 - t45922 + t45931 - t45933 + t45939 - t45946 - F::new(0.21450293971110256002e1) * t47500 * t13045 + F::new(0.23005755572352449806e2) * t2197 * t14385 + F::new(0.44688112439813033337e-1) * t45947 - F::new(0.89376224879626066674e-1) * t45950 - t45953 + F::new(0.63904876589867916127e-1) * t44133 + t47555 - t47558 - F::new(0.92023022289409799224e1) * t813 * t1445 * t2949 * t12161 - F::new(0.21450293971110256002e1) * t47496 * t13045 + t45958;
    t50302
}
