//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1294/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1294<F: Float>(t1650: F, t28752: F, t4468: F, t6159: F, t12844: F, t27583: F, t28806: F, t18120: F, t27567: F, t7986: F, t98260: F, t98263: F, t98268: F, t99201: F, t99205: F, t99210: F, t99213: F, t99219: F) -> (F, F) {
    let t99224 = t6159 * t28752 * t1650 * t4468;
    let t99229 = F::new(0.7722800925925925926e-4) * t27583 * t12844 * t28806;
    let t99231 = F::new(0.77382407407407407407e-3) * t98260 - F::new(0.30952962962962962962e-2) * t98263 - F::new(0.61836467013888888889e-4) * t27567 * t99201 - F::new(0.30918233506944444444e-4) * t27567 * t99205 + F::new(0.41224311342592592592e-4) * t27567 * t99210 - F::new(0.46336805555555555556e-3) * t27583 * t99213 * t18120 - F::new(0.23168402777777777778e-3) * t27583 * t99205 - F::new(0.18534722222222222222e-2) * t99219 * t7986 + F::new(0.15459116753472222222e-4) * t27567 * t99224 + t99229 - F::new(0.51588271604938271604e-3) * t98268;
    (t99224, t99231)
}
