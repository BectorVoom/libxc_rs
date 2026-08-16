//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1679/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1679<F: Float>(t1243: F, t42859: F, t460: F, t43351: F, t471: F, t1234: F, t1269: F, t12732: F, t1280: F, t1285: F, t1287: F, t12966: F, t12987: F, t13127: F, t13129: F, t13156: F, t3552: F, t3787: F, t44639: F, t44778: F, t44843: F, t44845: F, t44878: F, t45329: F, t45406: F, t45584: F, t45609: F, t487: F, t489: F, t5463: F, t5465: F) -> F {
    let t45832 = t42859 * t1243;
    let t45833 = t460 * t45832;
    let t45834 = t43351 * t471;
    let t45838 = -F::cast_from(0.23707617058567841754e2_f64) * t12987 * t1280 * t44778 + F::cast_from(0.15805078039045227836e2_f64) * t44843 * t1280 * t44845 - F::cast_from(0.65854491829355115987e0_f64) * t1234 * t1280 * t44878 + F::cast_from(0.39512695097613069592e1_f64) * t3552 * t3787 + F::cast_from(0.52683593463484092788e1_f64) * t5463 * t44639 * t5465 + F::cast_from(0.15805078039045227836e2_f64) * t12966 * t13156 + F::cast_from(0.26341796731742046395e1_f64) * t1285 * t1269 * t12732 * t1287 + F::cast_from(0.65854491829355115987e0_f64) * t460 * t489 * t45406 + F::cast_from(0.65854491829355115987e0_f64) * t1285 * t487 * t45329 * t1287 + F::cast_from(0.26341796731742046395e1_f64) * t13127 * t45584 * t13129 - F::cast_from(0.65854491829355115987e0_f64) * t45833 * t45609 * t45834;
    t45838
}
