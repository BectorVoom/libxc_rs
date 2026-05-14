//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1172/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1172<F: Float>(t100800: F, t5611: F, t6449: F, t92429: F, t92433: F, t100634: F, t100753: F, t100763: F, t100777: F, t100784: F, t100785: F, t100789: F, t18: F, t22522: F, t22583: F, t22585: F, t22598: F, t22605: F, t22736: F, t22738: F, t22761: F, t25649: F, t25653: F, t25692: F, t25713: F, t25774: F, t25802: F, t3066: F, t423: F, t428: F, t5579: F, t58524: F, t6437: F, t72: F, t92278: F, t92314: F, t92440: F, t92476: F, t92482: F, t92710: F, t92897: F, t92899: F, t92957: F, t930: F, t93122: F, t93129: F, t93131: F, t93136: F, t93138: F) -> (F, F, F) {
    let t100801 = t5611 * t100800;
    let t100803 = t92429 * t6449;
    let t100806 = t92433 * t6449;
    let t100808 = 0.1134997482304526749e-1 * t5611 * t100806;
    let t100825 = -t100753 + 0.51074886703703703704e-1 * t22522 * t100634 * t423 * t18 * t428 - 0.10338048737805743098e-3 * t92440 * t6437 * t22598 + 0.10338048737805743098e-3 * t100763 * t6437 * t22605 + 0.29693535778629056444e-3 * t93136 * t22585 * t930 * t93138 + 0.3520097786805302698e-5 * t93129 * t22585 * t930 * t93131 - 0.98910212891072794759e-5 * t92897 * t92899 * t100777 - 0.29693535778629056444e-4 * t22583 * t92476 * t100777 - 0.29693535778629056444e-3 * t93122 * t100784 * t100785 - 0.29693535778629056444e-3 * t22583 * t92482 * t100789 + 0.3520097786805302698e-5 * t93129 * t25692 * t25713 * t3066 + 0.1979569051908603763e-3 * t22583 * t92957 * t100789 + 0.14187468528806584362e-2 * t100801 - 0.62424861526748971195e-1 * t5611 * t100803 + t100808 - 0.22983699016666666666e0 * t22761 * t5579 * t72 * t58524 + 0.24511020009968991684e-5 * t92314 * t22738 * t25649 - 0.12255510004984495842e-5 * t92278 * t22738 * t25653 - 0.24511020009968991684e-6 * t22736 * t22738 * t25802 - 0.20411767610277765552e-7 * t22736 * t92710 * t25774;
    (t100803, t100806, t100825)
}
