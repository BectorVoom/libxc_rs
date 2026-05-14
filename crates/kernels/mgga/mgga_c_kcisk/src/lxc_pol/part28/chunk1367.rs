//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1367/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1367<F: Float>(t1790: F, t7715: F, t112372: F, t7234: F, t116137: F, t1799: F, t9945: F, t34093: F, t34103: F, t6713: F, t32903: F, t35084: F, t35238: F, t9660: F, t35192: F, t116311: F, t116416: F, t116645: F, t116672: F, t116677: F, t116701: F, t17010: F, t17717: F, t17722: F, t22289: F, t22294: F, t2785: F, t33031: F, t34016: F, t34274: F, t7278: F, t9922: F, t9926: F) -> (F, F, F, F, F, F) {
    let t121477 = t7715 * t1790;
    let t121479 = t7234 * t112372 * t121477;
    let t121489 = t1799 * t116137 * t9945;
    let t121492 = t6713 * t34093 * t34103;
    let t121495 = t1799 * t32903 * t35084;
    let t121512 = t35238 * t9660;
    let t121514 = t35192 * t9660;
    let t121516 = 0.46296296296296296297e-2 * t33031 * t121479 + 0.27777777777777777778e-1 * t33031 * t7234 * t116645 * t22289 - 0.55555555555555555558e-1 * t116416 * t9922 - 0.88437037037037037034e-2 * t121489 - t116672 + 0.17687407407407407407e-1 * t121492 - 0.33163888888888888888e-2 * t121495 - 0.20833333333333333334e-1 * t17010 * t9926 * t2785 + 0.55555555555555555558e-1 * t7278 * t34274 * t2785 - 0.10802469135802469136e-1 * t33031 * t17717 * t116311 * t22289 + 0.89351851851851851851e-3 * t116677 + 0.18518518518518518519e-1 * t33031 * t17722 * t34016 * t22294 + 0.18518518518518518519e-1 * t121512 - 0.33950617283950617287e-1 * t121514 + t116701;
    (t121477, t121479, t121489, t121492, t121495, t121516)
}
