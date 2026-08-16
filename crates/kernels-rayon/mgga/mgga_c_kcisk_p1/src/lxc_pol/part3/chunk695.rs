//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 695/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk695(t1849: f64, t569: f64, t1310: f64, t10463: f64, t662: f64, t10441: f64, t10449: f64, t1776: f64, t1775: f64, t10781: f64, t10785: f64, t10787: f64, t10789: f64, t10795: f64, t10800: f64, t10804: f64, t10810: f64, t10814: f64, t10817: f64, t10821: f64, t10828: f64, t1773: f64, t1778: f64, t4989: f64, t5009: f64, t5013: f64, t5022: f64, t5026: f64, t5034: f64) -> (f64, f64) {
    let t10831 = 1.0_f64 / t569 / t1849;
    let t10832 = t1310 * t10831;
    let t10833 = t662 * t10463;
    let t10834 = t10833 * t10441;
    let t10835 = t10832 * t10834;
    let t10838 = t1776 * t10449;
    let t10839 = t1775 * t10838;
    let t10842 = -0.5397236614853195164e-1_f64 * t1773 * t10781 + 0.10794473229706390328e0_f64 * t10785 + 0.53972366148531951639e-1_f64 * t10787 - 0.35981577432354634426e-1_f64 * t10789 + t10795 + 0.32383419689119170984e0_f64 * t4989 * t5034 - 0.35981577432354634425e-1_f64 * t10800 - 0.71963154864709268852e-1_f64 * t5013 * t10804 - 0.10794473229706390328e0_f64 * t4989 * t5022 - 0.35981577432354634425e-1_f64 * t10810 - 0.1439263097294185377e0_f64 * t1773 * t10814 + 0.53972366148531951639e-1_f64 * t10817 * t1778 + 0.10794473229706390328e0_f64 * t1773 * t10821 + 0.53972366148531951639e-1_f64 * t4989 * t5026 + 0.71963154864709268852e-1_f64 * t4989 * t5009 + 0.35981577432354634425e-1_f64 * t10828 + 0.55971342672551653552e-1_f64 * t1773 * t10835 + 0.17990788716177317213e-1_f64 * t1773 * t10839;
    (t10832, t10842)
}
