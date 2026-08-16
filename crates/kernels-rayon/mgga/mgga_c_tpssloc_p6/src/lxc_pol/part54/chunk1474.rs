//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1474/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1474(t122609: f64, t122610: f64, t122623: f64, t122625: f64, t122627: f64, t122645: f64, t122656: f64, t122659: f64, t122662: f64, t122664: f64, t26880: f64, t27180: f64, t27888: f64, t33746: f64, t7218: f64, t7266: f64, t7806: f64, t8690: f64) -> f64 {
    let t125003 = -t26880 * t8690 - 2.0_f64 * t27180 * t7266 - 2.0_f64 * t27888 * t7806 + t33746 * t7218 - 2.0_f64 * t122609 - 2.0_f64 * t122610 - t122623 - t122625 - t122627 - t122645 + t122656 - t122659 - t122662 - t122664;
    t125003
}
