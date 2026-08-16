//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1305/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1305(t10179: f64, t3147: f64, t1209: f64, t22185: f64, t3819: f64, t889: f64, t27984: f64, t3139: f64, t898: f64, t11163: f64, t237: f64, t900: f64) -> (f64, f64, f64, f64) {
    let t31643 = 0.10526802520742363173e2_f64 * t3147 * t10179;
    let t31647 = 0.10526802520742363173e2_f64 * t22185 * t1209 * t3819 * t889;
    let t31650 = 0.51947577317044391277e2_f64 * t898 * t27984 * t3139;
    let t31651 = t237 * t11163;
    let t31653 = 0.5848223622634646207e0_f64 * t31651 * t900;
    (t31643, t31647, t31650, t31653)
}
