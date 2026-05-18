//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1074/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1074<F: Float>(t5412: F, t5400: F, t6262: F, t4544: F, t4528: F, t13034: F, t13043: F, t13044: F, t18413: F, t18414: F, t18415: F, t18416: F, t18417: F, t18418: F, t18419: F, t6300: F, t6886: F, t6890: F, t6899: F, t6902: F) -> (F, F, F, F, F, F) {
    let t18423 = t5412 / F::new(8.0);
    let t18424 = t5400 / F::new(8.0);
    let t18425 = t6262 / F::new(8.0);
    let t18426 = t4544 / F::new(8.0);
    let t18427 = F::new(2.0) * t4528;
    let t18429 = -t18413 - t6899 - t6890 - t18414 - t6886 + t18415 - t6300 - t13034 - t18416 - t6902 + t18417 + t18418 - t13044 + t13043 + t18419;
    (t18423, t18424, t18425, t18426, t18427, t18429)
}
