//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 771/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk771<F: Float>(t86: F, t18: F, t502: F, t112: F, t113: F, t12068: F, t1577: F, t1927: F, t1934: F, t3297: F, t3307: F, t5: F, t505: F, t7742: F, t989: F, t992: F) -> F {
    let t87 = F::cast_from(10000000.0_f64) <= t86;
    let t12081 = t502 * t18;
    let t12091 = piecewise3::<F>(t87, F::new(0.0), t5 * t12068 * t113 / F::new(4.0) + t5 * t3297 * t505 / F::new(2.0) + t5 * t989 * t1934 / F::new(4.0) + t5 * t1927 * t992 / F::new(4.0) - t5 * t12081 * t1577 - t5 * t112 * t1577 / F::new(2.0) + F::new(3.0) / F::new(2.0) * t5 * t3307 * t7742);
    t12091
}
