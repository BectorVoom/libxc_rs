//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1213/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1213<F: Float>(t33799: F, t8407: F, t1410: F, t157: F, t1658: F, t2143: F, t2146: F, t2152: F, t2331: F, t2341: F, t32001: F, t33726: F, t33818: F, t40735: F, t40738: F, t40740: F, t40746: F, t40749: F, t557: F, t7912: F, t7931: F, t7932: F, t8004: F, t9503: F, t9769: F, t9801: F) -> F {
    let t40764 = t33799 * t8407;
    let t40771 = F::cast_from(0.8673628188205199462e0_f64) * t40735 - F::cast_from(0.8673628188205199462e0_f64) * t40738 - F::cast_from(0.8673628188205199462e0_f64) * t7931 * t7932 * t40740 - F::cast_from(0.4336814094102599731e0_f64) * t2143 * t9769 - F::cast_from(0.13170898365871023197e1_f64) * t40746 - F::cast_from(0.17347256376410398924e1_f64) * t7931 * t7932 * t40749 + F::cast_from(0.65854491829355115987e0_f64) * t32001 - F::cast_from(0.52041769129231196772e1_f64) * t2146 * t8004 * t2341 * t1658 - F::cast_from(0.13170898365871023197e1_f64) * t33818 * t557 + F::cast_from(0.8673628188205199462e0_f64) * t7912 * t9801 + F::cast_from(0.4336814094102599731e0_f64) * t7912 * t9503 - F::cast_from(0.17347256376410398924e1_f64) * t40764 - t33726 + F::cast_from(0.8673628188205199462e0_f64) * t2146 * t2152 * t2331 * t1410 * t157;
    t40771
}
