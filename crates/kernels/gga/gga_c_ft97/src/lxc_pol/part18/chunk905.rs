//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 905/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk905<F: Float>(t53: F, t538: F, t72: F, t5591: F, t1355: F, t2043: F, t22767: F, t22807: F, t22811: F, t22814: F, t23691: F, t23789: F, t23792: F, t23796: F, t23806: F, t23810: F, t23812: F, t23817: F, t23825: F, t23828: F, t23832: F, t5802: F, t5813: F, t5814: F, t5838: F, t8833: F, t8838: F) -> (F, F, F, F) {
    let t23833 = t538 * t53;
    let t23834 = t72 * t23833;
    let t23835 = t5591 * t23834;
    let t23838 = 0.53342400874074074075e0 * t5813 * t22767 * t5814 - 0.66678001092592592595e-1 * t23789 + 0.45306850413028723348e0 * t8838 * t23792 + 0.24163653553615319118e1 * t2043 * t23796 + 0.48327307107230638237e1 * t5802 * t23691 - 0.45306850413028723348e0 * t8833 * t23792 - 0.24163653553615319118e1 * t1355 * t23796 - 0.45306850413028723348e0 * t5802 * t23806 - 0.21895580739717983994e1 * t23810 * t23812 + 0.88904001456790123461e-1 * t5838 * t22814 - 0.11113000182098765433e-1 * t23817 - 0.16669500273148148149e-1 * t5838 * t22807 - 0.22226000364197530865e-1 * t5838 * t22811 + 0.4833552354146973393e0 * t23825 * t23828 + 0.4833552354146973393e0 * t23832 * t23835;
    (t23833, t23834, t23835, t23838)
}
