# Batched compile sweep — summary

- effective jobs: `3`
- effective batch_size: `20`
- families: `lda`
- total packages swept: `43`
- pass-1 successes: `0`
- pass-2 successes: `12`
- failures: `1`
- total wall-clock: `1042.1 s`
- peak-of-peak RSS: `17292.8 MB`

## Per-family

| family | ok | pass-2 | fail |
|---|---|---|---|
| lda | 12 | 12 | 1 |

## Per-batch timing + RSS

| batch_index | package_count | wall_time_s | peak_rss_mb | all_ok |
|---|---|---|---|---|
| 0 | 20 | 1042.1 | 17292.8 | False |

## Failures

- `libxc-kernel-lda_c_pk09` (family `lda`, batch 0)

VERDICT: HALTED_AT_BATCH_0
