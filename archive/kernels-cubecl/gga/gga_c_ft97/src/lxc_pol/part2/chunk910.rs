//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 910/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk910<F: Float>(t255: F, t9952: F, t258: F, t9570: F, t13863: F, t2526: F, t992: F, t2607: F, t2606: F, t14048: F, t14052: F, t14055: F, t14060: F, t14064: F, t14068: F, t14072: F, t14077: F, t1901: F, t446: F, t9813: F, t9822: F, t9824: F, t9826: F, t9828: F) -> F {
    let t14080 = t9952 * t255;
    let t14081 = t258 * t9570;
    let t14082 = t14081 * t13863;
    let t14083 = t14080 * t14082;
    let t14086 = t992 * t2526;
    let t14087 = t2607 * t14086;
    let t14088 = t2606 * t14087;
    let t14091 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t9813 - F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t9822 - F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t9824 + t9826 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t9828 - t446 * t14048 / F::cast_from(9.0_f64) - t14052 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t446 * t14055 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t14060 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t446 * t14064 + t1901 * t14068 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t1901 * t14072 - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t1901 * t14077 - F::cast_from(10.0_f64) / F::cast_from(81.0_f64) * t1901 * t14083 + t1901 * t14088 / F::cast_from(9.0_f64);
    t14091
}
