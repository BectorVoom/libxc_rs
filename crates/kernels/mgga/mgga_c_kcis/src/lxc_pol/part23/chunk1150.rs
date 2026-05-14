//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1150/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1150<F: Float>(t3978: F, t7969: F, t5426: F, t99199: F, t1370: F, t27636: F, t27606: F, t6140: F, t1650: F, t28752: F, t4468: F, t6159: F, t12844: F, t27583: F, t28806: F, t18120: F, t27567: F, t7986: F, t98260: F, t98263: F, t98268: F, t99201: F, t99205: F) -> (F, F, F, F) {
    let t99208 = t3978 * t7969;
    let t99210 = t99208 * t5426 * t99199;
    let t99213 = t1370 * t27636;
    let t99219 = t27606 * t6140;
    let t99224 = t6159 * t28752 * t1650 * t4468;
    let t99229 = 0.7722800925925925926e-4 * t27583 * t12844 * t28806;
    let t99231 = 0.77382407407407407407e-3 * t98260 - 0.30952962962962962962e-2 * t98263 - 0.61836467013888888889e-4 * t27567 * t99201 - 0.30918233506944444444e-4 * t27567 * t99205 + 0.41224311342592592592e-4 * t27567 * t99210 - 0.46336805555555555556e-3 * t27583 * t99213 * t18120 - 0.23168402777777777778e-3 * t27583 * t99205 - 0.18534722222222222222e-2 * t99219 * t7986 + 0.15459116753472222222e-4 * t27567 * t99224 + t99229 - 0.51588271604938271604e-3 * t98268;
    (t99210, t99219, t99224, t99231)
}
