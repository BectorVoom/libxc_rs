//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 1221/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk1221<F: Float>(t25: F, t265: F, t394: F, t85243: F, t2064: F, t2250: F, t24380: F, t40: F, t607: F, t7131: F, t84795: F, t85187: F, t9258: F, t11122: F, t1877: F, t2057: F, t23792: F, t23810: F, t23813: F, t24191: F, t24339: F, t24344: F, t2522: F, t26563: F, t28: F, t4314: F, t6848: F, t7110: F, t7114: F, t83556: F, t83566: F, t83579: F, t83582: F, t83603: F, t83613: F, t83617: F, t83627: F, t83630: F, t84791: F, t85167: F, dens_threshold: F, rho0: F, zeta_threshold: F) -> (F, F) {
    let t26 = t25 <= zeta_threshold;
    let t115 = rho0 <= dens_threshold || t26;
    let t395 = t265 < t394;
    let t85244 = piecewise3::<F>(t395, F::cast_from(0.0_f64), t85243);
    let t85254 = piecewise3::<F>(t115, t84795 + t85187, t85244 * t40 / F::cast_from(2.0_f64) + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t24380 * t607 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t7131 * t2250 + t2064 * t9258 / F::cast_from(2.0_f64));
    let t85296 = -F::cast_from(3.0_f64) * t1877 * t24339 * t23810 + t1877 * t2057 * t11122 / F::cast_from(2.0_f64) - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t1877 * t7114 * t83630 + F::cast_from(9.0_f64) * t24191 * t83582 + t1877 * t85167 * t28 / F::cast_from(2.0_f64) + F::cast_from(9.0_f64) * t2522 * t7110 * t23792 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t1877 * t84791 * t6848 + F::cast_from(3.0_f64) * t1877 * t24344 * t83617 - F::cast_from(9.0_f64) / F::cast_from(2.0_f64) * t24191 * t83579 - F::cast_from(9.0_f64) * t24191 * t83556 + F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t2522 * t2057 * t83613 + F::cast_from(9.0_f64) * t4314 * t2057 * t83566 + F::cast_from(9.0_f64) * t26563 * t83627 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t1877 * t24339 * t23813 - F::cast_from(3.0_f64) / F::cast_from(2.0_f64) * t1877 * t7114 * t83603;
    (t85254, t85296)
}
