//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 1007/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk1007<F: Float>(t13142: F, t7416: F, t2365: F, t32215: F, t6111: F, t13019: F, t4614: F, t833: F, t11001: F, t2714: F, t2718: F, t33725: F, t955: F) -> (F, F, F, F, F, F) {
    let t44009 = t7416 * t13142;
    let t44010 = F::cast_from(0.15976219147466979032e-1_f64) * t44009;
    let t44012 = t6111 * t2365 * t32215;
    let t44018 = t833 * t4614 * t13019;
    let t44020 = t2714 * t11001;
    let t44022 = t2718 * t11001;
    let t44024 = t955 * t33725;
    (t44010, t44012, t44018, t44020, t44022, t44024)
}
