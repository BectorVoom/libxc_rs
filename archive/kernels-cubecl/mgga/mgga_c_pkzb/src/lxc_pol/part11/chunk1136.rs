//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1136/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1136<F: Float>(t2899: F, t5974: F, t9310: F, t774: F, t9315: F, t7736: F, t9320: F, t7742: F, t9324: F, t2922: F, t9567: F, t21787: F, t9263: F) -> (F, F, F, F, F, F) {
    let t25453 = t2899 * t5974 * t9310;
    let t25456 = t2899 * t774 * t9315;
    let t25459 = t7736 * t774 * t9320;
    let t25462 = t7742 * t774 * t9324;
    let t25485 = t2922 * t774 * t9567;
    let t25518 = t2899 * t21787 * t9263;
    (t25453, t25456, t25459, t25462, t25485, t25518)
}
