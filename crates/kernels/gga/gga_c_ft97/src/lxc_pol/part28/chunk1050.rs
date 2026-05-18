//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1050/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1050<F: Float>(t25709: F, t32152: F, t25718: F, t22563: F, t52: F, t7837: F, t100483: F, t136474: F, t136475: F, t137047: F, t22513: F, t22837: F, t25649: F, t25653: F, t25699: F, t25704: F, t25755: F, t25760: F, t25829: F, t25832: F, t32151: F, t32250: F, t44965: F, t58: F, t92354: F, t929: F, t93047: F, t93157: F) -> (F, F, F) {
    let t145416 = t32152 * t25709;
    let t145419 = t32152 * t25718;
    let t145436 = t7837 * t22563 * t52;
    let t145447 = -F::new(0.24041029937711879614e-5) * t44965 * t32250 * t22837 * t58 * t929 - F::new(0.22705522127871165896e-3) * t22513 * t145416 + F::new(0.15137014751914110597e-3) * t22513 * t145419 + F::new(0.23022991505793434254e-7) * t100483 * t92354 * t32151 * t25755 - F::new(0.22705522127871165896e-3) * t93047 * t32152 * t25760 + F::new(0.22705522127871165896e-3) * t93157 * t32152 * t25699 - F::new(0.22705522127871165896e-3) * t93047 * t32152 * t25704 - F::new(0.20676097475611486194e-4) * t145436 * t25829 - F::new(0.68872808489893002037e-5) * t145436 * t25832 - F::new(0.13649345781532662579e-3) * t137047 * t136475 * t25649 + F::new(0.20474018672298993868e-3) * t136474 * t136475 * t25653;
    (t145416, t145419, t145447)
}
