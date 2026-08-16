//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1001/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1001(t32066: f64, t32073: f64, t32080: f64, t32082: f64, t32084: f64, t33778: f64, t33783: f64, t33786: f64, t33789: f64, t33794: f64, t33798: f64, t33801: f64, t33804: f64, t7912: f64, t7935: f64, t9015: f64) -> f64 {
    let t33810 = -0.17347256376410398924e1_f64 * t33778 * t7935 + 0.65854491829355115987e0_f64 * t32066 + t33783 - t32073 - t33786 + 0.8673628188205199462e0_f64 * t33789 - t33794 + t33798 - t33801 - t33804 + 0.13170898365871023197e1_f64 * t32080 - 0.26341796731742046394e1_f64 * t32082 - 0.13170898365871023197e1_f64 * t32084 + 0.8673628188205199462e0_f64 * t7912 * t9015;
    t33810
}
