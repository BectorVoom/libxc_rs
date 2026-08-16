//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2150/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2150<F: Float>(t23097: F, t4234: F, t776: F, t815: F, t81877: F, t81883: F, t13176: F, t6620: F, t849: F, t81857: F, t81859: F, t81874: F, t87287: F, t87289: F, t87292: F, t87293: F, t87296: F, t87298: F, t87301: F, t87304: F, t87306: F, t87308: F, t87312: F) -> F {
    let t87316 = t23097 * t815 * t4234 * t776;
    let t87319 = F::cast_from(0.33643963411783659044e-4_f64) * t81877;
    let t87320 = F::cast_from(0.10541775202358879834e-2_f64) * t81883;
    let t87321 = t13176 * t6620;
    let t87322 = t87321 * t849;
    let t87324 = -F::cast_from(5.0_f64) / F::cast_from(192.0_f64) * t87287 + t87289 / F::cast_from(192.0_f64) + t87292 + F::cast_from(0.16956557559538964158e-1_f64) * t87293 - t87296 / F::cast_from(768.0_f64) - t87298 / F::cast_from(1536.0_f64) - t87301 - F::cast_from(35.0_f64) / F::cast_from(576.0_f64) * t81857 + F::cast_from(0.14130464632949136799e-2_f64) * t81859 - F::cast_from(35.0_f64) / F::cast_from(216.0_f64) * t87304 - F::cast_from(0.67826230238155856634e-1_f64) * t87306 - F::cast_from(0.16956557559538964158e-1_f64) * t87308 + F::cast_from(0.40372756094140390854e-3_f64) * t87312 + F::cast_from(0.24223653656484234512e-2_f64) * t87316 + F::cast_from(0.33643963411783659045e-4_f64) * t81874 + t87319 - t87320 - t87322 / F::cast_from(192.0_f64);
    t87324
}
