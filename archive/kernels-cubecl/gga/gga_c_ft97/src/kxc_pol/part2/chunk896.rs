//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 896/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk896<F: Float>(t2486: F, t754: F, t3893: F, t3899: F, t8392: F, t2372: F, t255: F, t1131: F, t761: F, t2579: F, t13832: F, t13836: F, t13840: F, t13844: F, t13849: F, t13854: F, t13860: F, t13865: F, t13869: F, t13872: F, t13875: F, t13876: F, t1901: F, t193: F, t3281: F, t446: F, t89: F) -> F {
    let t13879 = t2486 * t754;
    let t13880 = t13879 * t3893;
    let t13884 = F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t8392 * t3899;
    let t13885 = t2372 * t255;
    let t13886 = t761 * t1131;
    let t13887 = t13886 * t2579;
    let t13888 = t13885 * t13887;
    let t13891 = -F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t13832 - t446 * t13836 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t13840 + t89 * t193 * t13844 / F::cast_from(3.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t13849 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t13854 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t13860 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t1901 * t13865 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t3281 * t13869 - F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t13872 + t13875 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1901 * t13876 - F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t1901 * t13880 - t13884 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1901 * t13888;
    t13891
}
