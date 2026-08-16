//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 755/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk755<F: Float>(t3219: F, t379: F, t11854: F, t1876: F, t925: F, t8557: F, t110: F, t8216: F, t11064: F, t3214: F, t3271: F, t11826: F, t11829: F, t11833: F, t11839: F, t11843: F, t11846: F, t11849: F, t11851: F, t1901: F, t3281: F, t446: F, t8393: F, t8409: F) -> F {
    let t11855 = t3219 * t379;
    let t11856 = t11854 * t11855;
    let t11859 = t925 * t1876;
    let t11860 = t8557 * t11859;
    let t11863 = t8216 * t110;
    let t11864 = t11863 * t11064;
    let t11867 = t3214 * t379;
    let t11868 = t8557 * t11867;
    let t11871 = t3271 * t379;
    let t11872 = t8557 * t11871;
    let t11876 = -t11826 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t11829 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t11833 - F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t8393 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t11839 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t3281 * t11843 - F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t11846 + t11849 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t446 * t11851 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1901 * t11856 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t11860 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t1901 * t11864 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t11868 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t11872 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t8409;
    t11876
}
