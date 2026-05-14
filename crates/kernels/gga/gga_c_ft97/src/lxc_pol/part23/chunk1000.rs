//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1000/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1000<F: Float>(t1127: F, t27729: F, t5049: F, t6018: F, t2917: F, t4969: F, t6045: F, t3766: F, t6814: F, t1113: F, t213: F, t231: F, t10915: F, t4965: F, t4973: F, t17807: F, t24311: F, t27506: F, t27584: F, t27605: F, t27733: F, t30652: F, t30684: F, t30689: F, t30718: F, t30721: F, t30728: F, t30758: F, t30761: F, t30763: F, t3774: F, t3789: F, t6023: F, t6043: F, t6055: F, t6758: F, t6767: F, t6808: F, t6809: F, t6815: F, t6819: F, t6824: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t30769 = t27729 * t1127;
    let t30772 = t6018 * t5049;
    let t30775 = t2917 * t4969;
    let t30776 = t6045 * t30775;
    let t30779 = t3766 * t6814;
    let t30780 = t1113 * t213;
    let t30781 = t231 * t30780;
    let t30785 = t10915 * t4965;
    let t30786 = t6045 * t30785;
    let t30789 = t2917 * t4973;
    let t30790 = t6045 * t30789;
    let t30793 = -0.38306165027777777778e-1 * t6808 * t30718 + 0.38306165027777777778e-1 * t6043 * t6045 * t30721 + 0.5297955163169938709e-2 * t6815 * t30728 - 0.27568129967481981593e-3 * t3774 * t27584 * t6758 - 0.51690243689028715488e-5 * t3774 * t6023 * t30652 + 0.20429954681481481482e0 * t6808 * t27506 * t6809 - 0.20429954681481481482e0 * t6043 * t27506 * t6824 + 0.25845121844514357744e-4 * t3774 * t6023 * t30684 + 0.12255510004984495842e-5 * t17807 * t27605 * t6758 - 0.1721820212247325051e-5 * t3774 * t24311 * t30689 + 2.0 * t30758 - 0.38482339615903025572e-7 * t3789 * t30761 * t30763 - 4.0 * t27733 * t6767 - 4.0 * t3766 * t30769 - 2.0 * t3766 * t30772 + 0.12768721675925925926e-1 * t6055 * t30776 - 0.18164417702296932716e-2 * t30779 * t6819 * t30781 - 0.85124811172839506173e-2 * t6055 * t30786 - 0.6384360837962962963e-2 * t6055 * t30790;
    (t30775, t30776, t30779, t30780, t30781, t30785, t30786, t30789, t30790, t30793)
}
