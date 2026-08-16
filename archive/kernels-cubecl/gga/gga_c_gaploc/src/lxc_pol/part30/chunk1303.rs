//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1303/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1303<F: Float>(t1457: F, t2103: F, t32223: F, t32219: F, t11065: F, t5666: F, t28659: F, t10954: F, t10677: F, t10967: F, t1445: F, t1710: F, t2049: F, t2154: F, t28645: F, t28675: F, t28678: F, t28681: F, t28684: F, t317: F, t32186: F, t32191: F, t32371: F, t3464: F, t5771: F, t797: F, t813: F, t833: F) -> F {
    let t33416 = F::cast_from(0.71500979903700853338e0_f64) * t2103 * t1457 * t32223;
    let t33419 = F::cast_from(0.14300195980740170668e1_f64) * t2103 * t1457 * t32219;
    let t33421 = F::cast_from(0.2556195063594716645e1_f64) * t5666 * t11065;
    let t33429 = F::cast_from(0.12780975317973583226e0_f64) * t28659;
    let t33436 = t1457 * t10954;
    let t33444 = t33416 + t33419 + t33421 + t28645 + F::cast_from(0.11502877786176224903e2_f64) * t833 * t1445 * t32371 - F::cast_from(0.46011511144704899612e1_f64) * t813 * t1445 * t10677 * t1710 + t33429 + t28675 + t28678 + t28681 - t28684 - F::cast_from(0.21450293971110256002e1_f64) * t797 * t1457 * t32186 + F::cast_from(0.14300195980740170668e1_f64) * t2103 * t1457 * t32191 - F::cast_from(0.21450293971110256002e1_f64) * t2049 * t33436 + F::cast_from(0.14300195980740170668e1_f64) * t5771 * t10967 + F::cast_from(0.35750489951850426669e0_f64) * t2154 * t3464 * t317;
    t33444
}
