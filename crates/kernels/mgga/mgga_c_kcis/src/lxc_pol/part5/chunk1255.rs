//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1255/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1255<F: Float>(t20974: F, t5653: F, t4162: F, t4160: F, t1497: F, t6281: F, t11898: F, t4170: F, t833: F, t5662: F, t5627: F, t5632: F) -> (F, F, F, F, F, F) {
    let t20975 = t5653 * t20974;
    let t20976 = t4162 * t20975;
    let t20977 = t4160 * t20976;
    let t20979 = t6281 * t1497;
    let t20980 = t11898 * t20979;
    let t20981 = t4170 * t20980;
    let t20982 = t4160 * t20981;
    let t20984 = t6281 * t833;
    let t20985 = t5662 * t20984;
    let t20986 = t4170 * t20985;
    let t20987 = t4160 * t20986;
    let t20989 = t5632 * t5627;
    (t20977, t20979, t20982, t20984, t20987, t20989)
}
