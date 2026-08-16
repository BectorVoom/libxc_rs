//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 819/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk819<F: Float>(t2: F, t3408: F, t1985: F, t558: F, t1775: F, t3520: F, t11717: F, t3510: F, t2075: F, t3518: F, t12839: F, t12840: F, t12843: F, t12846: F, t12850: F, t12852: F, t12855: F, t462: F, t9178: F, t9179: F, t9188: F, t9190: F, t9202: F, t9205: F, t9207: F, t9209: F, t9241: F) -> F {
    let t12858 = t2 * t3408;
    let t12860 = t1985 * t12858 * t558;
    let t12864 = F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1775 * t3520;
    let t12865 = t11717 * t3510;
    let t12868 = t1985 * t3518 * t2075;
    let t12878 = t12839 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t462 * t12840 + t462 * t12843 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t462 * t12846 - t12850 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t9179 - F::cast_from(4.0_f64) / F::cast_from(27.0_f64) * t12852 - F::cast_from(6.0_f64) * t462 * t12855 + F::cast_from(4.0_f64) * t462 * t12860 - t12864 + F::cast_from(22.0_f64) / F::cast_from(9.0_f64) * t12865 + F::cast_from(2.0_f64) * t462 * t12868 - t9178 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t9209 + t9241 / F::cast_from(3.0_f64) - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t9190 - F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t9202 + t9205 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(27.0_f64) * t9207 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t9188;
    t12878
}
