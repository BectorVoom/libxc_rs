//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1434/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1434(t224: f64, t32721: f64, t32741: f64, t33983: f64, t35243: f64, t11142: f64, t617: f64, t32099: f64, t32102: f64, t33952: f64, t33961: f64, t33966: f64, t33968: f64, t33974: f64, t33979: f64, t33981: f64, t33986: f64, t33997: f64, t34006: f64, t34008: f64, t34012: f64, t34018: f64, t34023: f64, t35239: f64, t35240: f64) -> f64 {
    let t35246 = t224 * (t32721 + t32741 + t33983 + t35243);
    let t35247 = t617 * t11142;
    let t35249 = t32099 - t32102 + t35246 + t33952 + t33961 + t33966 - t33968 - t33974 - t33979 - t33981 + t33986 + t33997 - t34006 + t34008 + t34012 + 2.0_f64 * t35247 - t34018 + t34023 - t35239 - t35240;
    t35249
}
