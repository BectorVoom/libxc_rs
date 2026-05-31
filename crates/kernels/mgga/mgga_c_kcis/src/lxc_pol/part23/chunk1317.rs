//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1317/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1317<F: Float>(t1299: F, t1885: F, t2233: F, t27716: F, t446: F, t449: F, t6260: F, t91791: F, t91793: F, t91863: F, t91866: F, t91869: F, t91872: F, t91874: F, t97548: F, t99737: F, t99738: F) -> F {
    let t99743 = -t91791 - t91793 - t91863 + t91866 - t446 * t1885 * t27716 / F::cast_from(16.0_f64) - t91869 + t91872 - t91874 + t97548 - t2233 * t1299 * t6260 / F::cast_from(8.0_f64) - t446 * t449 * (t99737 + t99738) / F::cast_from(16.0_f64);
    t99743
}
