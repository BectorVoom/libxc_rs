//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2048/2369 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2048<F: Float>(t87443: F, t25064: F, t81902: F, t23077: F, t6646: F, t6590: F, t23033: F, t25155: F, t6546: F, t25112: F, t81835: F, t23083: F, t25116: F) -> (F, F, F, F, F, F, F) {
    let t87444 = F::cast_from(0.40372756094140390854e-3_f64) * t87443;
    let t87445 = t81902 * t25064;
    let t87447 = t23077 * t6646;
    let t87451 = t6590 * t6646;
    let t87463 = t6546 * t23033 * t25155;
    let t87464 = F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t87463;
    let t87477 = t81835 * t25112;
    let t87478 = F::cast_from(0.16956557559538964159e-1_f64) * t87477;
    let t87487 = t23083 * t25116;
    (t87444, t87445, t87447, t87451, t87464, t87478, t87487)
}
