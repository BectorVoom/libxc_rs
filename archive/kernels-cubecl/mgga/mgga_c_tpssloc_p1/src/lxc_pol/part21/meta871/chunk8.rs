//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3208/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3208<F: Float>(t1243: F, t65955: F, t11881: F, t11904: F, t1247: F, t1249: F, t15000: F, t15016: F, t15241: F, t1756: F, t18572: F, t19142: F, t19157: F, t19180: F, t19203: F, t23508: F, t3507: F, t3604: F, t3610: F, t3612: F, t3628: F, t44691: F, t44785: F, t475: F, t494: F, t5064: F, t5072: F, t52447: F, t6168: F, t6252: F, t6256: F, t65347: F, t66662: F) -> F {
    let t66787 = t65955 * t1243;
    let t66802 = F::cast_from(2.0_f64) * t18572 * t1249 + t6168 * t3628 + F::cast_from(2.0_f64) * t5064 * t15016 + t66662 * t494 + F::cast_from(4.0_f64) * t3604 * t19180 + F::cast_from(8.0_f64) * t11904 * t19142 + F::cast_from(2.0_f64) * t5064 * t15241 - F::cast_from(12.0_f64) * t44691 * t19157 + F::cast_from(12.0_f64) * t11881 * t6256 * t15000 + F::cast_from(2.0_f64) * t66787 * t1247 - t44785 * t6252 * t23508 * t3507 * t475 + F::cast_from(2.0_f64) * t3610 * t65347 * t3612 + F::cast_from(8.0_f64) * t3610 * t5072 * t19203 + F::cast_from(2.0_f64) * t52447 * t1756;
    t66802
}
