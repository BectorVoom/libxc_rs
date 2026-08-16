//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2144/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2144<F: Float>(t22690: F, t23171: F, t25319: F, t2613: F, t4291: F, t7535: F, t81697: F, t81704: F, t81717: F, t829: F, t87609: F, t87613: F, t87615: F, t87619: F, t87620: F, t87627: F, t87630: F, t87633: F, t87635: F, t87640: F, t87645: F, t87650: F) -> F {
    let t87653 = t23171 * t22690 * t25319;
    let t87656 = F::cast_from(0.16449340668482264365e-1_f64) * t87609 - t87613 + F::cast_from(0.49348022005446793095e-1_f64) * t87615 + t87619 - F::cast_from(2.0_f64) * t4291 * t87620 * t829 + F::cast_from(0.19190897446562641759e-1_f64) * t81697 - F::cast_from(0.82246703342411321825e-2_f64) * t87627 - F::cast_from(0.49348022005446793095e-1_f64) * t87630 + F::cast_from(0.16449340668482264365e-1_f64) * t87633 - F::cast_from(0.12793931631041761173e0_f64) * t87635 + F::cast_from(0.19190897446562641759e-1_f64) * t81704 + F::cast_from(0.49348022005446793095e-1_f64) * t87640 - F::cast_from(0.19739208802178717238e0_f64) * t87645 - F::cast_from(0.16449340668482264365e-1_f64) * t87650 - F::cast_from(0.82246703342411321824e-2_f64) * t87653 + t81717 + t2613 * t7535;
    t87656
}
