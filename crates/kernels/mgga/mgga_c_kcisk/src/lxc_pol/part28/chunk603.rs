//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 603/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk603<F: Float>(t2394: F, t4761: F, t1980: F, t4638: F, t4724: F, t4769: F, t4776: F, t6756: F, t6761: F, t6766: F, t6769: F, t6778: F, t6780: F, t6818: F, t6820: F, t6823: F, t6826: F, t6829: F, t6832: F) -> (F, F, F) {
    let t6856 = t4761 * t2394;
    let t6857 = t6856 * t1980;
    let t6874 = -0.1294625e1 * t6778 + 0.258925e1 * t6780 + t4769 + 0.10064166666666666667e0 * t4638 + 0.10064166666666666667e0 * t6756 - 0.20128333333333333333e0 * t6761 + 0.60385e0 * t6766 + 0.60385e0 * t6769 + 0.82524375e-1 * t6818 + 0.16504875e0 * t6820 + t4776 + 0.5519e-1 * t4724 + 0.5519e-1 * t6823 - 0.27595e-1 * t6826 + 0.16557e0 * t6829 + 0.16557e0 * t6832;
    (t6856, t6857, t6874)
}
