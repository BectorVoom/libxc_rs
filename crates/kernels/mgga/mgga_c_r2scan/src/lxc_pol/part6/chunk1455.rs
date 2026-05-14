//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 1455/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk1455<F: Float>(t18995: F, t19013: F, t23741: F, t23742: F, t23745: F, t23748: F, t23750: F, t23751: F, t23752: F, t23753: F, t23757: F, t19032: F, t19037: F, t19041: F, t19048: F, t19057: F, t23759: F, t23761: F, t23763: F, t23765: F, t23769: F, t23773: F) -> (F, F) {
    let t27432 = t23741 - t23742 + t18995 + t23745 - t23748 + t23750 + t23751 - t23752 + t19013 + t23753 - t23757;
    let t27434 = -t23759 - t23761 + t23763 + t19032 - t23765 + t19037 + t23769 - t19041 - t19048 + t23773 - t19057;
    (t27432, t27434)
}
