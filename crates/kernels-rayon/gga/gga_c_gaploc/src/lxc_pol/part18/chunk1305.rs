//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1305/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1305(t1457: f64, t2103: f64, t32223: f64, t32219: f64, t11065: f64, t5666: f64, t28659: f64, t10954: f64, t10677: f64, t10967: f64, t1445: f64, t1710: f64, t2049: f64, t2154: f64, t28645: f64, t28675: f64, t28678: f64, t28681: f64, t28684: f64, t317: f64, t32186: f64, t32191: f64, t32371: f64, t3464: f64, t5771: f64, t797: f64, t813: f64, t833: f64) -> f64 {
    let t33416 = 0.71500979903700853338e0_f64 * t2103 * t1457 * t32223;
    let t33419 = 0.14300195980740170668e1_f64 * t2103 * t1457 * t32219;
    let t33421 = 0.2556195063594716645e1_f64 * t5666 * t11065;
    let t33429 = 0.12780975317973583226e0_f64 * t28659;
    let t33436 = t1457 * t10954;
    let t33444 = t33416 + t33419 + t33421 + t28645 + 0.11502877786176224903e2_f64 * t833 * t1445 * t32371 - 0.46011511144704899612e1_f64 * t813 * t1445 * t10677 * t1710 + t33429 + t28675 + t28678 + t28681 - t28684 - 0.21450293971110256002e1_f64 * t797 * t1457 * t32186 + 0.14300195980740170668e1_f64 * t2103 * t1457 * t32191 - 0.21450293971110256002e1_f64 * t2049 * t33436 + 0.14300195980740170668e1_f64 * t5771 * t10967 + 0.35750489951850426669e0_f64 * t2154 * t3464 * t317;
    t33444
}
