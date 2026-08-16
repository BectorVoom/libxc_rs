//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1305/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1305<F: Float>(t10179: F, t3147: F, t1209: F, t22185: F, t3819: F, t889: F, t27984: F, t3139: F, t898: F, t11163: F, t237: F, t900: F) -> (F, F, F, F) {
    let t31643 = F::cast_from(0.10526802520742363173e2_f64) * t3147 * t10179;
    let t31647 = F::cast_from(0.10526802520742363173e2_f64) * t22185 * t1209 * t3819 * t889;
    let t31650 = F::cast_from(0.51947577317044391277e2_f64) * t898 * t27984 * t3139;
    let t31651 = t237 * t11163;
    let t31653 = F::cast_from(0.5848223622634646207e0_f64) * t31651 * t900;
    (t31643, t31647, t31650, t31653)
}
