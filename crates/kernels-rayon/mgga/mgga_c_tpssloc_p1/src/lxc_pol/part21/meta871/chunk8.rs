//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3208/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3208(t1243: f64, t65955: f64, t11881: f64, t11904: f64, t1247: f64, t1249: f64, t15000: f64, t15016: f64, t15241: f64, t1756: f64, t18572: f64, t19142: f64, t19157: f64, t19180: f64, t19203: f64, t23508: f64, t3507: f64, t3604: f64, t3610: f64, t3612: f64, t3628: f64, t44691: f64, t44785: f64, t475: f64, t494: f64, t5064: f64, t5072: f64, t52447: f64, t6168: f64, t6252: f64, t6256: f64, t65347: f64, t66662: f64) -> f64 {
    let t66787 = t65955 * t1243;
    let t66802 = 2.0_f64 * t18572 * t1249 + t6168 * t3628 + 2.0_f64 * t5064 * t15016 + t66662 * t494 + 4.0_f64 * t3604 * t19180 + 8.0_f64 * t11904 * t19142 + 2.0_f64 * t5064 * t15241 - 12.0_f64 * t44691 * t19157 + 12.0_f64 * t11881 * t6256 * t15000 + 2.0_f64 * t66787 * t1247 - t44785 * t6252 * t23508 * t3507 * t475 + 2.0_f64 * t3610 * t65347 * t3612 + 8.0_f64 * t3610 * t5072 * t19203 + 2.0_f64 * t52447 * t1756;
    t66802
}
