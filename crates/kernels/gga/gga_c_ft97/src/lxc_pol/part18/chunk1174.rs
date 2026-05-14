//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 1174/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk1174<F: Float>(t3099: F, t373: F, t384: F, t100504: F, t100508: F, t100512: F, t100542: F, t100592: F, t100620: F, t100725: F, t100737: F, t100741: F, t100834: F, t100838: F, t100843: F, t100848: F, t100850: F, t100880: F, t100881: F, t1625: F, t1685: F, t22534: F, t22552: F, t22597: F, t22603: F, t22619: F, t22638: F, t22738: F, t22743: F, t22761: F, t22819: F, t22826: F, t25: F, t25643: F, t25779: F, t25793: F, t3066: F, t5538: F, t5540: F, t5579: F, t58531: F, t72: F, t73: F, t92278: F, t92440: F, t92441: F, t92715: F, t93268: F, t938: F) -> (F, F) {
    let t100892 = t3099 * t373 * t384;
    let t100899 = 0.15322466011111111111e0 * t22552 * t5579 * t72 * t58531 - 0.93019603785751168e-1 * t93268 * t25643 * t25 * t3066 - 0.10338048737805743098e-3 * t92440 * t92441 * t100834 - 0.44540303667943584666e-3 * t22619 * t73 * t100838 + 0.3443640424494650102e-5 * t100843 * t100737 * t100542 + 0.85124811172839506174e-2 * t100848 - 0.93019603785751168e-2 * t22826 * t100850 * t1625 + 0.1721820212247325051e-5 * t22603 * t22743 * t100592 + 0.76612330055555555556e-1 * t22552 * t5579 * t72 * t938 * t1685 - 0.11491849508333333333e0 * t22761 * t5579 * t72 * t100725 - 0.51690243689028715488e-5 * t5538 * t5540 * t100620 + 0.10330921273483950306e-5 * t5538 * t22743 * t100504 - 0.1721820212247325051e-5 * t5538 * t22743 * t100508 + 0.28677218675336554254e-7 * t5538 * t92715 * t100512 + t100880 - 0.29673063867321838428e-4 * t22534 * t73 * t100881 + 0.51690243689028715488e-4 * t22597 * t5540 * t100741 - 0.10595910326339877418e-1 * t22819 * t22638 * t25793 - 0.51690243689028715488e-4 * t22603 * t5540 * t100892 - 0.12255510004984495842e-5 * t92278 * t22738 * t25779;
    (t100892, t100899)
}
