//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 405/1308 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk405<F: Float>(t213: F, t218: F, t1729: F, t1819: F, t1920: F, t2009: F, t215: F, t690: F, t211: F, t414: F, t88: F, t220: F, t694: F, t43: F, zeta_threshold: F) -> (F, F) {
    let t214 = t213 <= zeta_threshold;
    let t219 = t218 <= zeta_threshold;
    let t2011 = t1729 + t1819 + t1920 + t2009;
    let t2013 = t215 * t215;
    let t2014 = F::new(1.0) / t2013;
    let t2015 = t690 * t690;
    let t2018 = t211 * t414;
    let t2020 = -F::new(2.0) * t88 + F::new(2.0) * t2018;
    let t2024 = piecewise3::<F>(t214, F::new(0.0), F::new(4.0) / F::new(9.0) * t2014 * t2015 + F::new(4.0) / F::new(3.0) * t215 * t2020);
    let t2025 = t220 * t220;
    let t2026 = F::new(1.0) / t2025;
    let t2027 = t694 * t694;
    let t2030 = -t2020;
    let t2034 = piecewise3::<F>(t219, F::new(0.0), F::new(4.0) / F::new(9.0) * t2026 * t2027 + F::new(4.0) / F::new(3.0) * t220 * t2030);
    let t2036 = (t2024 + t2034) * t43;
    (t2011, t2036)
}
