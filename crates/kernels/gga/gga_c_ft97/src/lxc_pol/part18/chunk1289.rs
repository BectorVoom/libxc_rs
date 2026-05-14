//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1289/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1289<F: Float>(t22515: F, t23833: F, t34876: F, t22511: F, t32772: F, t3392: F, t23826: F, t2001: F, t94507: F, t5818: F, t1008: F, t358: F, t100586: F, t93048: F, t100708: F, t104690: F, t104692: F, t104695: F, t104701: F, t104704: F, t104712: F, t12381: F, t1647: F, t23715: F, t26721: F, t26723: F, t379: F, t5570: F, t5838: F, t94395: F, t94401: F, t94514: F, t94535: F) -> (F, F, F) {
    let t104716 = t22515 * t34876 * t23833;
    let t104721 = t32772 * t22511;
    let t104722 = t3392 * t104721;
    let t104724 = t22515 * t34876 * t23826;
    let t104727 = t2001 * t94507;
    let t104732 = t5818 * t104721;
    let t104735 = t1008 * t358;
    let t104737 = t93048 * t104735 * t100586;
    let t104740 = 0.66678001092592592594e-1 * t23715 * t5570 * t26721 * t1647 + 0.18122740165211489339e1 * t104690 * t104692 - 0.66678001092592592594e-1 * t23715 * t5570 * t104695 * t379 + 0.96671047082939467857e0 * t94401 * t104701 - 0.66678001092592592594e-1 * t104704 * t26723 - 0.96671047082939467857e0 * t94514 * t104701 - 0.33339000546296296298e-1 * t5838 * t100708 - 0.22226000364197530865e-1 * t104712 + 0.1611184118048991131e0 * t94395 - 0.96671047082939467857e0 * t94514 * t104716 + 0.96671047082939467857e0 * t94401 * t104716 - 0.14500657062440920178e1 * t104722 * t104724 - 0.12002040196666666667e1 * t104727 * t5570 * t34876 * t12381 + 0.14500657062440920178e1 * t104732 * t104724 - 0.1611184118048991131e0 * t94535 * t104737;
    (t104735, t104737, t104740)
}
